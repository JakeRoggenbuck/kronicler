from fastapi import FastAPI
import uvicorn
import kronicler

app = FastAPI()


# You need to wrap helper functions
@kronicler.capture
def foo():
    return {"Hello": "World"}

# You cannot wrap routes right now
@app.get("/")
def read_root():
    return foo()

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
