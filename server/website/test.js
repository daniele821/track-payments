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

function buildTableFromJSON(json) {
    if (!json || !json.res || !json.res.query || !json.res.attributes) {
        console.error("Invalid JSON structure");
        return "";
    }

    const attributes = json.res.attributes;
    const data = json.res.query;

    // Create the table element
    let table = "<table border='1'><thead><tr>";

    // Add table headers
    attributes.forEach(attr => {
        table += `<th>${attr}</th>`;
    });
    table += "</tr></thead><tbody>";

    // Add table rows
    data.forEach(row => {
        table += "<tr>";
        row.forEach(cell => {
            table += `<td>${cell}</td>`;
        });
        table += "</tr>";
    });

    table += "</tbody></table>";
    return table;
}



// SEND DATA AS JSON
function populate(table) {
    fetch("", {
        method: "POST",
        body: JSON.stringify({
            "type": `select-${table}`,
            "data": {},
        }),
        headers: {
            'Content-Type': 'application/json'
        },
    })
        .then(res => res.json())
        .then(res => document.querySelector(`div#${table}`).innerHTML = buildTableFromJSON(res))
}

populate("city")
populate("shop")
populate("item")
populate("detail")
populate("payment")
populate("method")
