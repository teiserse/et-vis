
var loadData = function(){
                $.ajax({
                  type: 'GET',
                  contentType: 'application/json; charset=utf-8',
                  url: '/visual/index',
                  dataType: 'json',
                  success: function(data){
                    drawVis(data);
                  },
                  failure: function(result){
                    error();
                  }
                });
              };

function error() {
    console.log("Something went wrong!");
}

//var example = [{"x_axis": 100, "y_axis": 100, "radius": 30, "color": "red"}];

var centre_x = 500;
var centre_y = 300;

function drawVis(data){

  var container = d3.select("#plot-container")
    .append("svg")
    .attr("width", centre_x * 2)
    .attr("height",centre_y * 2);

  var centreRadius = Math.sqrt(data[0]['viewers']);
  var prevRadius = Math.sqrt(data[1]['viewers']);
  var prevAngle = 0;

  var circle1 = container.append("circle")
    .attr("cx",centre_x)
    .attr("cy",centre_y)
    .attr("r",centreRadius)
    .attr("fill","#b7410e")
    .append("svg:title")
    .text(data[0]['repo']);

  var circle2 = container.append("circle")
    .attr("cx",centre_x + centreRadius + prevRadius)
    .attr("cy",centre_y)
    .attr("r",prevRadius)
    .attr("fill","#b7950e")
    .append("svg:title")
    .text(data[1]['repo']);

  for (var index = 2; index < 30; index++){
    var newRadius = Math.sqrt(data[index]['viewers']);
    var currentAngle = Math.acos(
      (Math.pow(centreRadius + newRadius, 2) + Math.pow(centreRadius + prevRadius, 2) - Math.pow(prevRadius + newRadius, 2)) 
      /(2 * (centreRadius + newRadius) * (centreRadius + prevRadius)));
    
    prevAngle = prevAngle - currentAngle;

    var newCircle = container.append("circle")
      .attr("cx",centre_x + ((centreRadius + newRadius) * Math.cos(prevAngle)))
      .attr("cy",centre_y + ((centreRadius + newRadius) * Math.sin(prevAngle)))
      .attr("r", newRadius)
      .attr("fill",index % 2 == 0?'#b7410e':'#b7950e')
      .append("svg:title")
      .text(data[index]['repo']);

    prevRadius = newRadius;
  }

};

$(document).ready(function(){ 
  loadData(); 
});
