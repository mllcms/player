import{d as g,e as d,f as h,c as l,a as o,t as c,u as r,F as y,r as k,g as x,o as i,n as C,h as b,b as I}from"./index-88b91617.js";import{a as B}from"./video-8ae1474e.js";const F={class:"video-container video-container-scale"},S=["src"],T=o("select",{name:"multiplier",id:"multiplier"},[o("option",{value:"1.75"},"1.75")],-1),V=[T],$=["onClick"],E=g({__name:"video",setup(q){const v=x(),m=I(),u=d(v.params.name),e=B(),a=d();h(async()=>{var t;await e.request(u.value),(t=document.querySelector("nav div.active"))==null||t.scrollIntoView()});const p=()=>{const t=a.value.currentTime;if(!a.value||t<1)return;const n=a.value.duration||9999,s={index:t,len:n,scale:t/n*100};e.select.progress=s},_=()=>{const t=e.data.findIndex(s=>e.select==s),n=e.data[t+1];n&&(e.select=n)},f=()=>{a.value&&(a.value.currentTime=e.select.progress.index)};return(t,n)=>(i(),l("div",F,[o("h2",null,c(u.value),1),o("button",{onClick:n[0]||(n[0]=s=>r(m).back())},"返回"),o("video",{ref_key:"video",ref:a,class:"video",src:r(e).select.path,onEnded:_,onTimeupdate:p,onLoadstart:f,controls:"",autoplay:"",preload:"auto"},V,40,S),o("nav",null,[(i(!0),l(y,null,k(r(e).data,s=>(i(),l("div",{class:C(["item",{active:s==r(e).select}]),onClick:w=>r(e).select=s,style:b({backgroundImage:`linear-gradient(to right, #449DFC ${s.progress.scale}%,transparent 0%)`})},[o("p",null,c(s.name),1)],14,$))),256))]),o("h3",null,c(r(e).select.name),1)]))}});export{E as default};
