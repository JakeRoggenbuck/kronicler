from fastapi import FastAPI, Request
import uvicorn
import kronicler
import random
from fastapi.middleware.cors import CORSMiddleware
import time

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.middleware("http")
async def add_process_time_header(request: Request, call_next):
    call_next()

    response = await call_next(request)
    return response


DB = kronicler.Database(sync_consume=True)


# You need to wrap helper functions
@kronicler.capture
def foo():
    return {"Hello": "World"}


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
    return DB.fetch_all_as_dict()


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
