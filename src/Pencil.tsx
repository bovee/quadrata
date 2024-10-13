import { ReactNode } from 'react';


export function Pencil(props: {pencil: string, midPencil: boolean, highlightNumber: number}) {
  let {highlightNumber, midPencil, pencil} = props;

  if (midPencil) {
    let pencilSpans = pencil.split('').map(i => (
      <span className={`${parseInt(i) === highlightNumber ? 'highlight' : ''}`} key={i}>{i}</span>
    ));
    return (<div className="pencil" style={{top: 0, bottom: 0, height: '12px'}}>
      {pencilSpans}
    </div>);
  }

  // let pencilSpans = pencil.split('').map(i => (
  //   <span className={`${parseInt(i) === highlightNumber ? 'highlight' : ''}`} key={i}>{i}</span>
  // ));
  // let pencilOverflow: ReactNode[] = [];
  // if (pencil.length > 4) {
  //   pencilOverflow = pencilSpans.slice(4);
  //   pencil = pencil.slice(0, 4);
  // }
  // return (<>
  //   <div className="pencil" style={{top: 0}}>
  //     {pencilSpans}
  //   </div>
  //   <div className="pencil" style={{bottom: 0}}>
  //     {pencilOverflow}
  //   </div>
  // </>);

  // TODO: adjust to grid size

  let pencilValues = new Set(pencil.split(''));
  let pencilSpans: ReactNode[] = [];
  for (const i of ['1', '2', '3', '4', '5', '6', '7', '8', '9']) {
    pencilSpans.push(<span className={`${parseInt(i) === highlightNumber ? 'highlight' : ''}`} key={i}>
      {pencilValues.has(i) ? i : ''}
    </span>);
  }
  return (<>
    <div className="pencil" style={{top: 0}}>
      {pencilSpans}
    </div>
  </>);

}
