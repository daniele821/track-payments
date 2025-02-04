// SEND DATA AS FORM PARAMS
//const data = new FormData()
//data.append(1, 2)
//data.append(3, 4)
//
//document.querySelector("button").addEventListener("click", function(event) {
//    fetch("", {
//        method: "POST",
//        body: data
//    })
//        .then(res => res.text())
//        .then(res => console.log(res))
//})
//

// SEND DATA AS JSON
document.querySelector("button").addEventListener("click", function(event) {
    fetch("", {
        method: "POST",
        body: JSON.stringify({
            "type": "select-city",
            "data": {},
        }),
        headers: {
            'Content-Type': 'application/json'
        },
    })
        .then(res => res.text())
        .then(res => console.log(res))
})

