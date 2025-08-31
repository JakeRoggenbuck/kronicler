import threading
import logfrog
import time

@logfrog.capture
def worker(name):
    print(f"{name} starting...")
    time.sleep(2)  # wait for 1 second
    print(f"{name} finished!")

threads = []

for x in range(20):
    t = threading.Thread(target=worker, args=(f"Thread {x}",))

    t.start()
    threads.append(t)

for t in threads:
    t.join()

print("All threads done!")
