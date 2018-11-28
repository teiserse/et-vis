var loadData = function(){
                $.ajax({
                  type: 'GET',
                  contentType: 'application/json; charset=utf-8',
                  url: '/visual',
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

function drawVis(data){};

$(document).ready(function(){ 
  loadData(); 
});
