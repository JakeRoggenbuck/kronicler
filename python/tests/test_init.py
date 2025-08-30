def test_hello():
    assert 1 == 1


def test_wrapper(capsys):
    from logfrog import decorator_example

    @decorator_example
    def foo(): pass

    foo()

    stdout = capsys.readouterr()

    assert stdout.out == "LogFrog start...\nLogFrog end...\n"


def test_capture():
    from logfrog import capture

    # from .logfrog import LFQueue
    # LFQ = LFQueue()

    @capture
    def foo(): pass

    # LFQ.drop()

    # assert LFQ.empty()

    foo()

    # assert not LFQ.empty()
