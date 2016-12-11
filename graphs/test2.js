function test2_init(id){
  d3.select('#' + id).append('svg').append('path');
}

var test2Data = {};

function test2(id, value){
  console.log(!test2Data[id]);
  if (!test2Data[id]){
    test2Data[id] = [];
    for(i = 0; i < 100; i++){
      test2Data[id].push(0);
    }
  }
 
  test2Data[id].shift();
  test2Data[id].push(value);

  var data = test2Data[id];

  var svg = d3.select('#' + id).select('svg');
  var path = svg.select('path');

  var width = $('#' + id).children().width(),
      height = $('#' + id).children().height();
  
  var y = d3.scaleLinear()
    .range([height, 0])
    .domain([0, 1]);

  var x = d3.scaleLinear()
    .range([0, width])
    .domain([data.length, 0]);

  var lineChart = d3.line()
                    .x(function(d, i){ return x(i) })
                    .y(function(d){ return y(d) });

  path.attr("d", lineChart(data));
}
