import kronicler

# If you want a sync consume database
kronicler.DB = kronicler.Database(True)

@kronicler.capture
def foo():
    pass

foo()
