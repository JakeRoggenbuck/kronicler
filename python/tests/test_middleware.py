import pytest
from fastapi import FastAPI
from fastapi.testclient import TestClient
import kronicler
from kronicler import (
    KroniclerFunctionMiddleware,
    KroniclerEndpointMiddleware,
    KroniclerMiddleware,
)
import kronicler as kr
import os
import shutil


@pytest.fixture(autouse=True)
def clean_database():
    """Fixture to ensure a clean database state for each test."""
    # Clean up any existing database files before test
    db_path = ".kronicler_data"
    if os.path.exists(db_path):
        raise Exception(db_path)
        #shutil.rmtree(db_path)
    yield
    # Cleanup after test
    if os.path.exists(db_path):
        #shutil.rmtree(db_path)
        raise Exception(db_path)


def test_kronicler_function_middleware_captures_function_name():
    """Test that KroniclerFunctionMiddleware captures function names."""
    app = FastAPI()
    app.add_middleware(KroniclerFunctionMiddleware)

    @app.get("/test-endpoint")
    def test_function():
        return {"message": "test"}

    @app.get("/another-endpoint")
    def another_function():
        return {"message": "another"}

    client = TestClient(app)

    # Make requests
    response1 = client.get("/test-endpoint")
    assert response1.status_code == 200

    response2 = client.get("/another-endpoint")
    assert response2.status_code == 200

    # Check that logs were captured
    logs = kr.DB.logs()
    assert len(logs) >= 2

    # Verify function names were captured
    function_names = [log[1] if isinstance(log, list) and len(log) > 1 else None for log in logs]
    assert "test_function" in function_names
    assert "another_function" in function_names


def test_kronicler_endpoint_middleware_captures_endpoint_path():
    """Test that KroniclerEndpointMiddleware captures endpoint paths."""
    app = FastAPI()
    app.add_middleware(KroniclerEndpointMiddleware)

    @app.get("/test-endpoint")
    def test_function():
        return {"message": "test"}

    @app.get("/another-endpoint")
    def another_function():
        return {"message": "another"}

    client = TestClient(app)

    # Make requests
    response1 = client.get("/test-endpoint")
    assert response1.status_code == 200

    response2 = client.get("/another-endpoint")
    assert response2.status_code == 200

    # Check that logs were captured
    logs = kr.DB.logs()
    assert len(logs) >= 2

    # Verify endpoint paths were captured
    endpoint_names = [log[1] if isinstance(log, list) and len(log) > 1 else None for log in logs]
    assert "/test-endpoint" in endpoint_names
    assert "/another-endpoint" in endpoint_names


def test_kronicler_middleware_deprecated_but_works():
    """Test that deprecated KroniclerMiddleware still works (extends KroniclerFunctionMiddleware)."""
    import warnings

    app = FastAPI()

    # Should show deprecation warning
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        app.add_middleware(KroniclerMiddleware)
        assert len(w) == 1
        assert issubclass(w[0].category, DeprecationWarning)
        assert "deprecated" in str(w[0].message).lower()

    @app.get("/test")
    def test_function():
        return {"message": "test"}

    client = TestClient(app)

    # Make request
    response = client.get("/test")
    assert response.status_code == 200

    # Check that logs were captured
    logs = kr.DB.logs()
    assert len(logs) >= 1

    # Verify function name was captured (since KroniclerMiddleware extends KroniclerFunctionMiddleware)
    function_names = [log[1] if isinstance(log, list) and len(log) > 1 else None for log in logs]
    assert "test_function" in function_names


def test_middleware_captures_timing():
    """Test that middleware captures timing information."""
    import time

    app = FastAPI()
    app.add_middleware(KroniclerFunctionMiddleware)

    @app.get("/slow-endpoint")
    def slow_function():
        time.sleep(0.1)  # Simulate some work
        return {"message": "slow"}

    client = TestClient(app)

    # Make request
    response = client.get("/slow-endpoint")
    assert response.status_code == 200

    # Check that logs were captured
    logs = kr.DB.logs()
    assert len(logs) >= 1

    # Verify timing data exists (start and end times)
    log = logs[-1]  # Get the most recent log
    if isinstance(log, list) and len(log) >= 4:
        # Format: [id, function_name, start_time, delta]
        start_time = log[2]
        delta = log[3]
        assert start_time > 0
        assert delta > 0
        # Delta should be at least 0.1 seconds (100ms) in nanoseconds
        assert delta >= 100_000_000  # 0.1 seconds in nanoseconds


def test_middleware_handles_multiple_requests():
    """Test that middleware handles multiple requests correctly."""
    app = FastAPI()
    app.add_middleware(KroniclerFunctionMiddleware)

    @app.get("/counter")
    def counter():
        return {"count": 1}

    client = TestClient(app)

    # Make multiple requests
    for _ in range(5):
        response = client.get("/counter")
        assert response.status_code == 200

    # Check that all requests were captured
    logs = kr.DB.logs()
    assert len(logs) >= 5

    # Verify all logs are for the same function
    function_names = [log[1] if isinstance(log, list) and len(log) > 1 else None for log in logs[-5:]]
    assert all(name == "counter" for name in function_names if name is not None)


def test_middleware_handles_different_http_methods():
    """Test that middleware works with different HTTP methods."""
    app = FastAPI()
    app.add_middleware(KroniclerFunctionMiddleware)

    @app.get("/get-endpoint")
    def get_handler():
        return {"method": "GET"}

    @app.post("/post-endpoint")
    def post_handler():
        return {"method": "POST"}

    @app.put("/put-endpoint")
    def put_handler():
        return {"method": "PUT"}

    client = TestClient(app)

    # Make requests with different methods
    assert client.get("/get-endpoint").status_code == 200
    assert client.post("/post-endpoint").status_code == 200
    assert client.put("/put-endpoint").status_code == 200

    # Check that all requests were captured
    logs = kr.DB.logs()
    assert len(logs) >= 3

    function_names = [log[1] if isinstance(log, list) and len(log) > 1 else None for log in logs[-3:]]
    assert "get_handler" in function_names
    assert "post_handler" in function_names
    assert "put_handler" in function_names


def test_middleware_handles_route_with_path_params():
    """Test that middleware works with routes that have path parameters."""
    app = FastAPI()
    app.add_middleware(KroniclerFunctionMiddleware)

    @app.get("/items/{item_id}")
    def get_item(item_id: int):
        return {"item_id": item_id}

    client = TestClient(app)

    # Make request with path parameter
    response = client.get("/items/123")
    assert response.status_code == 200

    # Check that request was captured
    logs = kr.DB.logs()
    assert len(logs) >= 1

    # Verify function name was captured
    function_names = [log[1] if isinstance(log, list) and len(log) > 1 else None for log in logs]
    assert "get_item" in function_names

