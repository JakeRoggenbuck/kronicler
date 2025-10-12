from fastapi import FastAPI
import uvicorn
import kronicler

app = FastAPI()

DB = kronicler.Database(sync_consume=True)
app.add_middleware(kronicler.KroniclerMiddleware)


@app.get("/")
def read_root():
    return {"Hello": "World - middleware"}


@app.get("/logs")
def read_logs():
    return DB.logs()


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
