import kronicler

kronicler.database_init()

@kronicler.capture
def foo():
    pass

foo()

import time
time.sleep(2)
