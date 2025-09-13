from fastapi import FastAPI
import uvicorn
import kronicler

app = FastAPI()
DB = kronicler.Database()


# You need to wrap helper functions
@kronicler.capture
def foo():
    return {"Hello": "World"}


# You cannot wrap routes right now
@app.get("/")
def read_root():
    return foo()


@app.get("/logs")
def read_logs():
    return DB.fetch_all_as_dict()


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
