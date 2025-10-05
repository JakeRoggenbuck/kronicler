import kronicler

# If you want a sync consume database
kronicler.DB = kronicler.Database(sync_consume=True)


@kronicler.capture
def foo():
    pass


foo()
