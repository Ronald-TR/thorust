(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[931],{12353:function(e,t,n){Promise.resolve().then(n.bind(n,76964))},76964:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return L}});var o,r,i=n(57437),d=n(2265),l=n(39227),a=n(41135),s=n(73724),c=n(63955),u=n(64867),h=n(48967),p=n(59673);n(28473);var g=n(61553),f=n(35511),x=n(80432),b=n(34273),Z=n(8819),m=n(75816),k=n(67264),j=n.n(k);(o=r||(r={})).Completed="Completed",o.Failed="Failed",o.Skipped="Skipped",o.NotStarted="NotStarted";let S={Running:{node:f.Z[100],edge:f.Z[500],border:f.Z[500],animated:!0},Completed:{node:x.Z[100],edge:x.Z[500],border:x.Z[500],animated:!1},Failed:{node:b.Z[100],edge:b.Z[500],border:b.Z[500],animated:!0},NotStarted:{node:Z.Z[100],edge:Z.Z[500],border:Z.Z[700],animated:!0},Skipped:{node:m.Z[100],edge:m.Z[500],border:m.Z[500],animated:!0},Default:{node:f.Z[100],edge:f.Z[500],border:f.Z[500],animated:!0}};function v(e){switch(e){case"Completed":return S.Completed;case"Failed":return S.Failed;case"Skipped":return S.Skipped;case"NotStarted":return S.NotStarted;case"Running":return S.Running;default:return S.Default}}function C(e){let t=e.split("-"),n=t.pop()||"",o=t.join("-");return{label:o,status:n}}let y=new(j()).graphlib.Graph;y.setDefaultEdgeLabel(()=>({}));let E=function(e,t){let n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:"LR",o=arguments.length>3&&void 0!==arguments[3]?arguments[3]:[];e.forEach(e=>{e.hidden=!(o.includes(e.status)||0===o.length)}),t.forEach(e=>{e.hidden=!(o.includes(e.source.status)||o.includes(e.target.status)||0===o.length)});let r="LR"===n;return y.setGraph({rankdir:n}),e.forEach(e=>{y.setNode(e.id,{width:172,height:36})}),t.forEach(e=>{y.setEdge(e.source,e.target)}),j().layout(y),e.forEach(e=>{let t=y.node(e.id);return e.targetPosition=r?"left":"top",e.sourcePosition=r?"right":"bottom",e.position={x:t.x-86,y:t.y-18},e}),{nodes:e,edges:t}};var w=n(43226);function R(e){let{nodes:t,edges:n,setNodes:o,setEdges:r}=e,[c,u]=d.useState([]),h=e=>{let i=c.includes(e)?c.filter(t=>t!==e):[...c,e],{nodes:d,edges:l}=E(t,n,"LR",i);u(i),o([...d]),r([...l])};return(0,i.jsx)(a.Z,{size:"small","aria-label":"small button group",children:Object.entries(S).filter(e=>{let[t]=e;return"Default"!==t}).map(e=>{let[n,o]=e;return(0,i.jsx)(s.Z,{onClick:()=>h(n),style:{color:o.border,backgroundColor:c.includes(n)?o.node:"white"},startIcon:(0,i.jsx)(l.Z,{sx:{width:17,height:17,backgroundColor:o.border},children:(0,i.jsx)(w.Z,{sx:{color:o.node,fontSize:12,fontWeight:"bold"},children:t.filter(e=>e.status===n).length})}),children:n},n)})})}var _=e=>{let{dot:t}=e,[n,o,r]=(0,u.Rr)([]),[l,c,f]=(0,u.ll)([]),x=(0,d.useCallback)(e=>c(t=>(0,u.Z_)({...e,type:u.t8.SmoothStep,animated:!0},t)),[]),b=(0,d.useCallback)(e=>{let{nodes:t,edges:r}=E(n,l,e);o([...t]),c([...r])},[n,l]);return(0,d.useEffect)(()=>{let e=(0,g.read)(t),n=e.nodes().map((t,n)=>{let o;let r=e.node(t),{label:i,status:d}=C(r.label);return{id:t,status:d,data:{label:i},type:"default",style:{background:(o=v(C(r.label).status)).node,color:"dark",border:"2px solid ".concat(o.border)},position:{x:0,y:0}}}),r=e.edges().map(t=>{let n;return{id:"".concat(t.v,"-").concat(t.w),source:t.v,target:t.w,animated:(n=v(C(e.node(t.v).label).status)).animated,type:"smoothstep",style:{stroke:n.edge}}}),{nodes:i,edges:d}=E(n,r);o([...i]),c([...d])},[t]),(0,i.jsxs)(u.x$,{nodes:n,edges:l,onNodesChange:r,onEdgesChange:f,onConnect:x,connectionLineType:u.t8.SmoothStep,fitView:!0,onNodeClick:(e,t)=>console.log(e,t),children:[(0,i.jsxs)(u.s_,{position:"bottom-right",children:[(0,i.jsx)(w.Z,{variant:"overline",display:"block",children:"layout orientation"}),(0,i.jsxs)(a.Z,{orientation:"vertical",variant:"outlined","aria-label":"outlined button group",children:[(0,i.jsx)(s.Z,{onClick:()=>b("TB"),children:"vertical"}),(0,i.jsx)(s.Z,{onClick:()=>b("LR"),children:"horizontal"})]})]}),(0,i.jsx)(u.s_,{position:"top-left",children:(0,i.jsx)(R,{nodes:n,edges:l,setNodes:o,setEdges:c})}),(0,i.jsx)(h.Z,{}),(0,i.jsx)(p.A,{})]})},N=n(84889);function L(){let[e,t]=d.useState("info"),[n,o]=d.useState(""),[r,u]=d.useState(!1),h=async()=>{fetch("".concat(N.api.BASE_URL,"/dot")).then(e=>e.text()).then(e=>{o(e)})};d.useEffect(()=>{let e=setInterval(async()=>{h(),u(n.includes("Running")),n.includes("Failed")&&t("error")},500);return h(),()=>clearInterval(e)},[n]);let p=async()=>{t("warning"),fetch("".concat(N.api.BASE_URL,"/runner/all"))},g=async()=>{fetch("".concat(N.api.BASE_URL,"/runner/reset")).then(()=>{h(),t("info")})};return(0,i.jsx)(l.Z,{sx:{display:"flex",width:"100%"},children:(0,i.jsxs)(l.Z,{sx:{width:"100%"},children:[(0,i.jsxs)(a.Z,{children:[(0,i.jsx)(s.Z,{disabled:r,color:e,variant:"outlined",onClick:()=>p(),children:"Run workflow"}),(0,i.jsx)(s.Z,{disabled:r,color:"info",variant:"outlined",onClick:()=>g(),children:"Reset"})]}),(0,i.jsx)(c.Z,{maxWidth:"lg",sx:{},children:(0,i.jsx)(l.Z,{sx:{height:"80vh"},children:n.length>0&&(0,i.jsx)(_,{dot:n})})})]})})}},84889:function(e){"use strict";e.exports={api:{BASE_URL:"/api"}}}},function(e){e.O(0,[866,28,414,329,427,250,971,596,744],function(){return e(e.s=12353)}),_N_E=e.O()}]);