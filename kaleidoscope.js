window.addEventListener("load", function(){
    var c = document.getElementById("c");
    var baseImg = document.getElementById("base");
    var baseRImg = document.getElementById("baseR");
    var ctx = c.getContext("2d");
    var pat = ctx.createPattern(baseImg, "repeat");
    var patR = ctx.createPattern(baseRImg, "repeat");
    var patDim = 150; //pattern is 150x150 square.
    var SqrtOf3_4 = Math.sqrt(3)/2;
    //height of triangle side given side length of 150 is:
    var height =  SqrtOf3_4 * patDim;
var offset = 0;
ctx.translate(-0.5*patDim, 0);
var fn = function(alternateMode){
    offset = (offset - 1) % 1024
    var i = 0;


    //draw kaleidoscope first row.
    ctx.save();
    ctx.fillStyle=pat;
    ctx.translate(0, offset);
    while(i <= 3){
      ctx.beginPath();
      ctx.moveTo(0,-offset);
      ctx.lineTo(patDim, -offset);
      ctx.lineTo(0.5*patDim, height-offset);
      ctx.closePath();
      ctx.fill();
      if(i%3==0){
        ctx.translate(patDim,-offset);
        ctx.rotate(-120*Math.PI/180);
        ctx.translate(-patDim,offset);
      }
      else if(i%3==1){
        if(alternateMode){
          ctx.rotate(120*Math.PI/180);
          ctx.translate(-3*patDim, 0);
          ctx.rotate(-120*Math.PI/180);
        }
        ctx.translate(0.5*patDim, height-offset);
        ctx.rotate(-120*Math.PI/180);
        ctx.translate(-0.5*patDim, -height+offset);
      }
      else if(i%3==2){
        ctx.translate(0,-offset);
        ctx.rotate(-120*Math.PI/180);
        ctx.translate(0,offset);
      }
      i++;
    }
    ctx.restore();
    ctx.save();
    ctx.scale(-1,-1);
    ctx.fillStyle=patR;
    ctx.translate((-i+(i%3==0?0.5:i%3==1?1.5:-0.5))*patDim, -height+offset);
    ctx.translate(0, -offset);
    ctx.rotate(120*Math.PI/180);
    ctx.translate(0, offset);
    var j=0;
    while(j < i+1){
      ctx.beginPath();
      if(j>0||!alternateMode){
        ctx.moveTo(0,-offset);
        ctx.lineTo(patDim, -offset);
        ctx.lineTo(0.5*patDim, height-offset);
        ctx.closePath();
        ctx.fill();
      }
      if(j%3==1){
        ctx.translate(patDim,-offset);
        ctx.rotate(-120*Math.PI/180);
        ctx.translate(-patDim,offset);
      }
      else if(j%3==2){
        ctx.translate(0.5*patDim, height-offset);
        ctx.rotate(-120*Math.PI/180);
        ctx.translate(-0.5*patDim, -height+offset);
      }
      else if(j%3==0){
        ctx.translate(0,-offset);
        ctx.rotate(-120*Math.PI/180);
        ctx.translate(0,offset);
      }
      j++;
    }
    ctx.restore();
};
var patternHeight = Math.floor(SqrtOf3_4*patDim*2)
var tile = function(){
  var rowData = ctx.getImageData(0,0,patDim*3,patternHeight);
  for(var i=0; patternHeight*i<c.height+SqrtOf3_4*patDim; i++){
    for(var j = 0; j*patDim<c.width+patDim; j+=3){
      ctx.putImageData(rowData,j*patDim,i*patternHeight);
    }
  }
}

var tilingPatternData;
var target = document.getElementById("target");
window.setInterval(
  function() {
    fn(false);
    ctx.translate(1.5*patDim, height);
    fn(true);
    ctx.translate(-1.5*patDim, -height);
    tile();
  } , 1000/20);
  //};
});




