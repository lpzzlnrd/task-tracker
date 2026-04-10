from fastapi import FastAPI

app= FastAPI()

@app.get("/")
def inicio():
    return {"mensaje": "hola"}

@app.get("/docs")
def documentacion():
    return {"doc": "No hay"}

