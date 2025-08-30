def test_hello():
    assert 1 == 1


def test_wrapper(capsys):
    from logfrog import decorator_example

    @decorator_example
    def foo(): pass

    foo()

    stdout = capsys.readouterr()

    assert stdout.out == "LogFrog start...\nLogFrog end...\n"
