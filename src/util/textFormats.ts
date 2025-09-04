function formatTextLimit(text: string, limit: number){
  if(text.length > limit){
    return text.slice(0,limit-3)+"..."
  }else{
    return text;
  }
}

const formatTime = (time: number) => {
  const minutes = Math.floor(time / 60)
    .toString()
    .padStart(2, "0");
  const seconds = Math.floor(time % 60)
    .toString()
    .padStart(2, "0");
  return `${minutes}:${seconds}`;
};

export{formatTextLimit,formatTime}