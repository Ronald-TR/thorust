(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[289],{83614:function(e,n,i){Promise.resolve().then(i.bind(i,50423)),Promise.resolve().then(i.t.bind(i,69050,23)),Promise.resolve().then(i.t.bind(i,60371,23)),Promise.resolve().then(i.bind(i,3483))},3483:function(e,n,i){"use strict";i.r(n),i.d(n,{default:function(){return F}});var t=i(57437),r=i(2265),s=i(39227),l=i(20039),c=i(52653),o=i(73701),a=i(39279),d=i(30666),h=i(15795),x=i(66988),u=i(98489),j=i(43226),Z=i(29872),m=i(45695),p=i(38018),g=i(35266),f=i(23341),_=i(53688),b=i(6358),v=i(87780),E=i(48727),k=i(81344),y=i(44294),S=i(84889);function w(e){let{id:n,name:i,description:t,test_id:r,service:s,history:l}=e;return{id:n,name:i,description:t,test_id:r,service:s,history:l}}function D(e){var n;let{row:i}=e,[h,Z]=r.useState(!1);return(0,t.jsxs)(r.Fragment,{children:[(0,t.jsxs)(u.Z,{sx:{"& > *":{borderBottom:"unset"}},children:[(0,t.jsx)(d.Z,{children:(0,t.jsx)(c.Z,{"aria-label":"expand row",size:"small",onClick:()=>Z(!h),children:h?(0,t.jsx)(p.Z,{}):(0,t.jsx)(m.Z,{})})}),(0,t.jsx)(d.Z,{align:"left",children:i.test_id}),(0,t.jsx)(d.Z,{align:"center",children:i.name}),(0,t.jsx)(d.Z,{align:"center",children:i.description}),(0,t.jsx)(d.Z,{align:"center",children:function(e){if(0===e.length)return"-";let n=e[e.length-1];return["Completed","Failed"].includes(null==n?void 0:n.to_status)?"".concat(n.duration_millis.toFixed(0),"ms"):(console.log(e),"No running record")}(i.history)}),(0,t.jsx)(d.Z,{align:"center",children:(null===(n=i.history[i.history.length-1])||void 0===n?void 0:n.to_status)||"-"})]}),(0,t.jsx)(u.Z,{children:(0,t.jsx)(d.Z,{style:{paddingBottom:0,paddingTop:0},colSpan:6,children:(0,t.jsx)(l.Z,{in:h,timeout:"auto",unmountOnExit:!0,children:(0,t.jsxs)(s.Z,{sx:{margin:1},children:[(0,t.jsx)(j.Z,{variant:"h6",gutterBottom:!0,component:"div",children:"History"}),(0,t.jsxs)(o.Z,{size:"small","aria-label":"purchases",children:[(0,t.jsx)(x.Z,{children:(0,t.jsxs)(u.Z,{children:[(0,t.jsx)(d.Z,{children:"From"}),(0,t.jsx)(d.Z,{children:"To"}),(0,t.jsx)(d.Z,{align:"center",children:"Duration"}),(0,t.jsx)(d.Z,{align:"center",children:"From Date"}),(0,t.jsx)(d.Z,{align:"center",children:"To Date"})]})}),(0,t.jsx)(a.Z,{children:i.history.map(e=>(0,t.jsxs)(u.Z,{children:[(0,t.jsx)(d.Z,{component:"th",scope:"row",children:e.from_status}),(0,t.jsx)(d.Z,{children:e.to_status}),(0,t.jsxs)(d.Z,{align:"center",children:[(e.duration_millis/1e3).toFixed(2),"s"]}),(0,t.jsx)(d.Z,{align:"center",children:e.from_created_at}),(0,t.jsx)(d.Z,{align:"center",children:e.to_created_at})]},e.from_status))})]})]})})})})]})}function F(){let[e,n]=r.useState({}),[i,s]=r.useState([]),c=i=>{e[i]||n({[i]:!0}),n({[i]:!e[i]})},j=i.reduce((e,n)=>(e[n.service]||(e[n.service]=[]),e[n.service].push(n),e),{});return r.useEffect(()=>{fetch("".concat(S.api.BASE_URL,"/nodes")).then(e=>e.json()).then(e=>{s(e.map(e=>w(e)))})},[]),(0,t.jsx)(g.Z,{sx:{width:"100%",bgcolor:"background.paper"},subheader:(0,t.jsx)(f.Z,{component:"div",id:"nested-list-subheader",children:"Service Report"}),children:Object.entries(j).map(n=>{let[i,r]=n;return(0,t.jsxs)(g.Z,{children:[(0,t.jsxs)(_.Z,{onClick:()=>c(i),children:[(0,t.jsx)(b.Z,{children:(0,t.jsx)(y.Z,{})}),(0,t.jsx)(v.Z,{primary:i}),e?(0,t.jsx)(E.Z,{}):(0,t.jsx)(k.Z,{})]}),(0,t.jsx)(l.Z,{in:e[i],timeout:"auto",unmountOnExit:!0,children:(0,t.jsx)(h.Z,{component:Z.Z,children:(0,t.jsxs)(o.Z,{"aria-label":"collapsible table",children:[(0,t.jsx)(x.Z,{children:(0,t.jsxs)(u.Z,{children:[(0,t.jsx)(d.Z,{}),(0,t.jsx)(d.Z,{align:"left",children:"Test ID"}),(0,t.jsx)(d.Z,{align:"center",children:"Name"}),(0,t.jsx)(d.Z,{align:"center",children:"Description"}),(0,t.jsx)(d.Z,{align:"center",children:"Duration"}),(0,t.jsx)(d.Z,{align:"center",children:"Last status"})]})}),(0,t.jsx)(a.Z,{children:r.map(e=>(0,t.jsx)(D,{row:w(e)},e.name))})]})})},i)]},i)})})}},84889:function(e){"use strict";e.exports={api:{BASE_URL:"/api"}}}},function(e){e.O(0,[414,646,329,427,245,383,971,596,744],function(){return e(e.s=83614)}),_N_E=e.O()}]);