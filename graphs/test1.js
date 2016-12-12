var simulation;
var color;
var link;
var node;
var graph={nodes:[], links:[]};

function ticked() {
  link
      .attr("x1", function(d) { return d.source.x; })
      .attr("y1", function(d) { return d.source.y; })
      .attr("x2", function(d) { return d.target.x; })
      .attr("y2", function(d) { return d.target.y; });

  node
      .attr("cx", function(d) { return d.x; })
      .attr("cy", function(d) { return d.y; });
}

function test1_init(id){
  var svg = d3.select('#' + id).append('svg');
  svg.append("g").attr("class", "nodes");
  svg.append("g").attr("class", "links");

  var width = $('#' + id).children().width(),
      height = $('#' + id).children().height();

  color = d3.scaleOrdinal(d3.schemeCategory20);

  simulation = d3.forceSimulation()
      .force("link", d3.forceLink().id(function(d) { return d.id; }))
      .force("charge", d3.forceManyBody())
      .force("center", d3.forceCenter(width / 2, height / 2))
      .alphaTarget(1).on("tick", ticked);

  link = svg.select(".links")
    .selectAll("line");

  node = svg.select(".nodes")
    .selectAll("circle");

  node.append("title")
      .text(function(d) { return d.id; });

}

function test1(id, new_graph){

  var svg = d3.select('#' + id).select('svg');

  var new_nodes = new_graph.nodes.slice(graph.nodes.length);
  var new_links = new_graph.links.slice(graph.links.length);

  new_nodes.forEach(function(node){
    graph.nodes.push(node);
  });

  new_links.forEach(function(link){
    graph.links.push(link);
  });

  node = node.data(graph.nodes)
    .enter().append("circle")
      .attr("r", 5)
      .attr("fill", function(d) { return color(d.group); })
      .merge(node)
      .call(d3.drag()
          .on("start", dragstarted)
          .on("drag", dragged)
          .on("end", dragended));

  link = link.data(graph.links)
    .enter().append("line")
      .attr("stroke-width", function(d) { return Math.sqrt(d.value * 10); })
      .merge(link);

  simulation.nodes(graph.nodes);
  simulation.force("link").links(graph.links);
  simulation.alpha(1).restart();

  function dragstarted(d) {
    if (!d3.event.active) simulation.alphaTarget(0.3).restart();
    d.fx = d.x;
    d.fy = d.y;
  }

  function dragged(d) {
    d.fx = d3.event.x;
    d.fy = d3.event.y;
  }

  function dragended(d) {
    if (!d3.event.active) simulation.alphaTarget(0);
    d.fx = null;
    d.fy = null;
  }
}
