from fastapi import FastAPI
import uvicorn
import kronicler
import random
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

DB = kronicler.Database(sync_consume=True)


# You need to wrap helper functions
@kronicler.capture
def foo():
    return {"Hello": "World - fastapi"}


@kronicler.capture
def bar():
    values = [random.randint(0, 100) for _ in range(100)]

    return {"val": max(values)}


# You cannot wrap routes right now
@app.get("/")
def read_root():
    return foo()


@app.get("/val")
def get_value():
    return bar()


@app.get("/logs")
def read_logs():
    return DB.logs()


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8001)
