RustyDashed = {
  _gridItem: '<div class="grid-stack-item">\
                <div class="grid-stack-item-content">\
                  <svg style="width:90%;height:90%;"></svg>\
                </div>\
              </div>',
  grid: $('.grid-stack'),
  addGraph: function(path){
    $('<link/>', {
       rel: 'stylesheet',
       type: 'text/css',
       href: path + '.css'
    }).appendTo('head');
    $.getScript( path + '.js');
  },
  createGridItem: function(id, x, y, width, height){
    var gridItem = $(this._gridItem);
    gridItem.children().children().attr("id", id); 
    gridItem.attr('data-gs-x', x);
    gridItem.attr('data-gs-y', y);
    gridItem.attr('data-gs-width', width);
    gridItem.attr('data-gs-height', height);
    this.grid.append(gridItem);
  },
  init: function(options){
    $('.grid-stack').gridstack(options);
  }
}
