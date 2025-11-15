import pytest
from unittest.mock import Mock, patch, MagicMock
from starlette.applications import Starlette
from starlette.responses import JSONResponse
from starlette.testclient import TestClient
from starlette.routing import Route
import time
import warnings

# Import the module to test
from kronicler import (
    capture,
    KroniclerEndpointMiddleware,
    KroniclerFunctionMiddleware,
    KroniclerMiddleware,
    DB,
    KRONICLER_ENABLED,
)


class TestCaptureDecorator:
    """Tests for the @capture decorator"""

    @pytest.fixture(autouse=True)
    def mock_db_capture(self):
        """Mock DB.capture for all tests in this class"""
        with patch.object(DB, 'capture') as mock_capture:
            yield mock_capture

    def test_capture_decorator_calls_function(self, mock_db_capture):
        """Test that decorated function is called and returns correct value"""
        @capture
        def add_numbers(a, b):
            return a + b

        result = add_numbers(2, 3)
        assert result == 5

    def test_capture_decorator_records_timing(self, mock_db_capture):
        """Test that capture decorator records function execution"""
        @capture
        def test_func():
            return "result"

        result = test_func()

        assert result == "result"
        assert mock_db_capture.called
        assert mock_db_capture.call_count == 1

        # Verify the captured data
        call_args = mock_db_capture.call_args[0]
        assert call_args[0] == "test_func"  # function name
        assert isinstance(call_args[1], tuple)  # args
        assert isinstance(call_args[2], int)  # start time
        assert isinstance(call_args[3], int)  # end time
        assert call_args[3] > call_args[2]  # end > start

    def test_capture_decorator_with_args(self, mock_db_capture):
        """Test that decorator passes through function arguments"""
        @capture
        def multiply(x, y, z=1):
            return x * y * z

        result = multiply(2, 3, z=4)

        assert result == 24
        call_args = mock_db_capture.call_args[0]
        assert call_args[0] == "multiply"
        assert call_args[1] == (2, 3)  # positional args

    def test_capture_decorator_with_exception(self, mock_db_capture):
        """Test that exceptions are propagated correctly"""
        @capture
        def failing_func():
            raise ValueError("Test error")

        with pytest.raises(ValueError, match="Test error"):
            failing_func()

    @patch.dict('os.environ', {'KRONICLER_ENABLED': 'false'})
    def test_capture_disabled_when_env_false(self):
        """Test that capture is disabled when KRONICLER_ENABLED is false"""
        # Need to reload module to pick up env change
        import importlib
        import kronicler
        importlib.reload(kronicler)

        @kronicler.capture
        def test_func():
            return "result"

        result = test_func()
        assert result == "result"
        # DB.capture should not be called when disabled

    def test_capture_measures_actual_execution_time(self, mock_db_capture):
        """Test that capture measures real execution time"""
        @capture
        def slow_func():
            time.sleep(0.01)  # 10ms
            return "done"

        result = slow_func()

        assert result == "done"
        call_args = mock_db_capture.call_args[0]
        start_time = call_args[2]
        end_time = call_args[3]
        duration_ns = end_time - start_time

        # Should be at least 10ms (10,000,000 nanoseconds)
        assert duration_ns >= 10_000_000


class TestKroniclerEndpointMiddleware:
    """Tests for KroniclerEndpointMiddleware"""

    @pytest.fixture(autouse=True)
    def mock_db_capture(self):
        """Mock DB.capture for all tests in this class"""
        with patch.object(DB, 'capture') as mock_capture:
            yield mock_capture

    def test_endpoint_middleware_captures_request(self, mock_db_capture):
        """Test that middleware captures endpoint timing"""
        async def homepage(request):
            return JSONResponse({"message": "Hello"})

        app = Starlette(routes=[Route("/", homepage)])
        app.add_middleware(KroniclerEndpointMiddleware)

        client = TestClient(app)
        response = client.get("/")

        assert response.status_code == 200
        assert response.json() == {"message": "Hello"}

        # Verify capture was called
        assert mock_db_capture.called
        call_args = mock_db_capture.call_args[0]
        assert call_args[0] == "/"  # endpoint path
        assert call_args[1] == []  # empty args
        assert isinstance(call_args[2], int)  # start time
        assert isinstance(call_args[3], int)  # end time
        assert call_args[3] > call_args[2]

    def test_endpoint_middleware_multiple_endpoints(self, mock_db_capture):
        """Test middleware with multiple endpoints"""
        async def home(request):
            return JSONResponse({"page": "home"})

        async def about(request):
            return JSONResponse({"page": "about"})

        app = Starlette(routes=[
            Route("/", home),
            Route("/about", about),
        ])
        app.add_middleware(KroniclerEndpointMiddleware)

        client = TestClient(app)

        response1 = client.get("/")
        response2 = client.get("/about")

        assert response1.json() == {"page": "home"}
        assert response2.json() == {"page": "about"}

        # Should have captured both requests
        assert mock_db_capture.call_count == 2

        # Check captured endpoints
        first_call = mock_db_capture.call_args_list[0][0]
        second_call = mock_db_capture.call_args_list[1][0]

        assert first_call[0] == "/"
        assert second_call[0] == "/about"

    def test_endpoint_middleware_with_path_params(self, mock_db_capture):
        """Test middleware captures endpoints with path parameters"""
        async def user_detail(request):
            user_id = request.path_params["user_id"]
            return JSONResponse({"user_id": user_id})

        app = Starlette(routes=[
            Route("/users/{user_id}", user_detail),
        ])
        app.add_middleware(KroniclerEndpointMiddleware)

        client = TestClient(app)
        response = client.get("/users/123")

        assert response.json() == {"user_id": "123"}

        call_args = mock_db_capture.call_args[0]
        assert call_args[0] == "/users/123"  # Full path captured


class TestKroniclerFunctionMiddleware:
    """Tests for KroniclerFunctionMiddleware"""

    @pytest.fixture(autouse=True)
    def mock_db_capture(self):
        """Mock DB.capture for all tests in this class"""
        with patch.object(DB, 'capture') as mock_capture:
            yield mock_capture

    def test_function_middleware_captures_function_name(self, mock_db_capture):
        """Test that middleware captures function name"""
        async def my_endpoint(request):
            return JSONResponse({"status": "ok"})

        app = Starlette(routes=[Route("/test", my_endpoint)])
        app.add_middleware(KroniclerFunctionMiddleware)

        client = TestClient(app)
        response = client.get("/test")

        assert response.status_code == 200

        # Verify function name was captured
        call_args = mock_db_capture.call_args[0]
        assert call_args[0] == "my_endpoint"
        assert call_args[1] == []
        assert isinstance(call_args[2], int)
        assert isinstance(call_args[3], int)

    def test_function_middleware_multiple_functions(self, mock_db_capture):
        """Test middleware with different function names"""
        async def func_a(request):
            return JSONResponse({"func": "a"})

        async def func_b(request):
            return JSONResponse({"func": "b"})

        app = Starlette(routes=[
            Route("/a", func_a),
            Route("/b", func_b),
        ])
        app.add_middleware(KroniclerFunctionMiddleware)

        client = TestClient(app)
        client.get("/a")
        client.get("/b")

        assert mock_db_capture.call_count == 2

        first_call = mock_db_capture.call_args_list[0][0]
        second_call = mock_db_capture.call_args_list[1][0]

        assert first_call[0] == "func_a"
        assert second_call[0] == "func_b"

    def test_function_middleware_no_route_match(self, mock_db_capture):
        """Test middleware behavior when route has no endpoint"""
        app = Starlette(routes=[])
        app.add_middleware(KroniclerFunctionMiddleware)

        client = TestClient(app)
        response = client.get("/nonexistent")

        assert response.status_code == 404
        # Should not capture if no route matched


class TestKroniclerMiddlewareDeprecation:
    """Tests for deprecated KroniclerMiddleware"""

    @pytest.fixture(autouse=True)
    def mock_db_capture(self):
        """Mock DB.capture for all tests in this class"""
        with patch.object(DB, 'capture') as mock_capture:
            yield mock_capture

    def test_kronicler_middleware_shows_deprecation_warning(self, mock_db_capture):
        """Test that KroniclerMiddleware raises deprecation warning"""
        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")

            app = Starlette(routes=[])
            app.add_middleware(KroniclerMiddleware)

            # Check that a deprecation warning was issued
            assert len(w) == 1
            assert issubclass(w[0].category, DeprecationWarning)
            assert "KroniclerMiddleware is deprecated" in str(w[0].message)
            assert "KroniclerFunctionMiddleware" in str(w[0].message)

    def test_kronicler_middleware_still_works(self, mock_db_capture):
        """Test that deprecated middleware still functions correctly"""
        async def test_endpoint(request):
            return JSONResponse({"test": "ok"})

        app = Starlette(routes=[Route("/test", test_endpoint)])

        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            app.add_middleware(KroniclerMiddleware)

        client = TestClient(app)
        response = client.get("/test")

        assert response.status_code == 200
        assert mock_db_capture.called


class TestIntegration:
    """Integration tests combining decorator and middleware"""

    @pytest.fixture(autouse=True)
    def mock_db_capture(self):
        """Mock DB.capture for all tests in this class"""
        with patch.object(DB, 'capture') as mock_capture:
            yield mock_capture

    def test_decorator_and_middleware_together(self, mock_db_capture):
        """Test that both decorator and middleware can work together"""
        @capture
        def process_data(data):
            return data.upper()

        async def endpoint(request):
            result = process_data("hello")
            return JSONResponse({"result": result})

        app = Starlette(routes=[Route("/process", endpoint)])
        app.add_middleware(KroniclerEndpointMiddleware)

        client = TestClient(app)
        response = client.get("/process")

        assert response.json() == {"result": "HELLO"}

        # Should have captured both the decorator and middleware
        assert mock_db_capture.call_count == 2

        # First call should be from decorator
        first_call = mock_db_capture.call_args_list[0][0]
        assert first_call[0] == "process_data"

        # Second call should be from middleware
        second_call = mock_db_capture.call_args_list[1][0]
        assert second_call[0] == "/process"

    def test_performance_overhead_minimal(self, mock_db_capture):
        """Test that Kronicler adds minimal overhead"""
        async def fast_endpoint(request):
            return JSONResponse({"status": "ok"})

        app = Starlette(routes=[Route("/fast", fast_endpoint)])
        app.add_middleware(KroniclerEndpointMiddleware)

        client = TestClient(app)

        # Make multiple requests
        start = time.perf_counter()
        for _ in range(100):
            client.get("/fast")
        duration = time.perf_counter() - start

        # Should complete 100 requests in reasonable time (< 5 seconds)
        assert duration < 5.0
        assert mock_db_capture.call_count == 100


# Fixtures for common test setup
@pytest.fixture
def mock_db():
    """Fixture to provide a mocked DB instance"""
    with patch.object(DB, 'capture') as mock_capture:
        yield mock_capture


@pytest.fixture
def test_app():
    """Fixture to provide a basic test application"""
    async def index(request):
        return JSONResponse({"message": "index"})

    app = Starlette(routes=[Route("/", index)])
    return app


# Run tests with: pytest test_kronicler.py -v
