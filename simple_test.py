import logfrog

@logfrog.capture
def foo():
    pass

foo()
