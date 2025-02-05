#!/bin/python3

import server


# server.run()
def runOp(run):
    try:
        print(run())
    except Exception as e:
        print(e)


runOp(lambda: server.DB.insertCity("Cesena"))
runOp(lambda: server.DB.insertCity("Asti"))
runOp(lambda: server.DB.insertItem("Pane"))
runOp(lambda: server.DB.insertItem("Pizza"))
runOp(lambda: server.DB.insertItem("Briosche"))
runOp(lambda: server.DB.insertItem("Spremuta"))
runOp(lambda: server.DB.insertItem("caramelle"))
runOp(lambda: server.DB.insertMethod("Contante"))
runOp(lambda: server.DB.insertMethod("SanPaolo"))
runOp(lambda: server.DB.insertMethod("Postepay"))
runOp(lambda: server.DB.insertShop("Coop"))
runOp(lambda: server.DB.insertShop("Paninaro"))
runOp(lambda: server.DB.insertShop("Conad"))
runOp(lambda: server.DB.insertShop("Luka's Pizza"))
runOp(lambda: server.DB.insertPayment("-", "Cesena", "Coop", "Postepay"))
runOp(lambda: server.DB.insertPayment("-", "Cesena", "Luka's Pizza", "Postepay"))
runOp(lambda: server.DB.insertDetailOrder("Briosche", 1, 2, 1200))
runOp(lambda: server.DB.insertDetailOrder("Spremuta", 1, 1, 2900))
runOp(lambda: server.DB.insertDetailOrder("Pizza", 2, 1, 8900))

print(server.DB.checkCityExists("Cesena"))
print(server.DB.checkCityExists(""))
