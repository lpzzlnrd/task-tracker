from pydantic import BaseModel
from fastapi import FastAPI
from typing import Optional # nuevo import para manejar los campos que sean opcionales o acepten null

app= FastAPI()

class Usuaraio(BaseModel):

    id: int
    nombre: str
    email: str
    password: str #se muestra en las respuestas 


class Categoria(BaseModel):

    id: int
    nombre: str

class Producto(BaseModel):
    id: int
    nombre: str
    precio: float
    categoria_id: int #esta es la foreign key para la otra entidad 
    descripcion: Optional[str] = None #campo opcional para la descripcion



class Pedido(BaseModel):

    id: int
    usuario_id: int #foreign key para el usuario que hizo el pedido
    producto_id: int  #foreign key para el producto relacionado
    cantidad: int 


CATEGORIAS = [
    {"id": 1, "nombre": "Electrónica"},
    {"id": 2, "nombre": "Ropa"},
]

PRODUCTOS = [
    {"id": 1, "nombre": "Laptop", "precio": 999.99, "categoria_id": 1},
    {"id": 2, "nombre": "Camisa", "precio": 25.00,  "categoria_id": 2},
]

USUARIOS = [
    {"id": 1, "nombre": "Leo",  "email": "leo@example.com"},
    {"id": 2, "nombre": "Brian", "email": "brian@example.com"},
]

PEDIDOS = [
    {"id": 1, "usuario_id": 1, "producto_id": 1, "cantidad": 2},
    {"id": 2, "usuario_id": 2, "producto_id": 2, "cantidad": 1},
]



@app.get("/usuarios")

def getUsers():
    return USUARIOS


app.get("/usuarios/{id}")

def getUserById(id: int):

    for u in USUARIOS:
        if u["id"] == id:
            return u
    return {"error": "Usuario no encontrado"}



@app.get("categorias")

def getCategorias():
    return CATEGORIAS


@app.et("/productos")

def getProductos():
    return PRODUCTOS

@app.get("productos/{id}")

def getProductoById(id: int):
    for p in PRODUCTOS:
        if p["id"] == id:
            return p
    return {"error": "Producto no encontrado"}


@app.get("/pedidos/{id}")

def getPedido(id: int):
    for p in PEDIDOS:
        if p["id"] == id:
            return p
    return {"error": "Pedido no encontrado"}


@app.get("/usuarios/{id}/pedidos") #un join para un pedido de un usuario especifico

def PedidosUsuario(id: int):
    return [p for p in PEDIDOS if p["usuario_id"] == id]