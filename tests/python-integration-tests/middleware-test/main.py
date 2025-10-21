from fastapi import FastAPI, Request
import uvicorn
import kronicler

app = FastAPI()

DB = kronicler.Database(sync_consume=True)
app.add_middleware(kronicler.KroniclerMiddleware)


@app.get("/")
def read_root():
    return {"Hello": "World - middleware"}


@app.post("/print")
async def print_data(request: Request):
    data = await request.json()
    print("Received POST data:", data)
    return {"status": "success", "received": data}


@app.get("/logs")
def read_logs():
    return DB.logs()


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
