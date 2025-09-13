import kronicler
import time

db = kronicler.Database()
db.init()

@kronicler.capture
def foo():
    pass

foo()

time.sleep(5)
