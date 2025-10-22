def test_hello():
    assert 1 == 1


def test_wrapper(capsys):
    from kronicler import decorator_example

    @decorator_example
    def foo():
        pass

    foo()

    stdout = capsys.readouterr()

    assert stdout.out == "Kronicler start...\nKronicler end...\n"


def test_capture():
    from kronicler import capture

    # from .kronicler import KQueue
    # KQ = KQueue()

    @capture
    def foo():
        pass

    # KQ.drop()

    # assert KQ.empty()

    foo()

    # assert not KQ.empty()
