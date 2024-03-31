// Find better way of doing this:
var distributor_list = '{"tcp_distributors": [{"id": 3,"ip_address": "127.0.0.1:4999","up_to_date": true, "reliable": false}]}';

function show_dist_list() {
    var json = JSON.parse(distributor_list);
    var tcp_dists = json.tcp_distributors; 
    
    document.getElementById("tcp_dists").innerHTML = "<pre>" + JSON.stringify(tcp_dists, null, 2) + "</pre>";
}
