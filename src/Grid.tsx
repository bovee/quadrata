import {createRef, useReducer, useState, Dispatch, ReactNode, RefObject, SetStateAction} from 'react';

import { Puzzle } from '../quadrata/pkg/quadrata';
import { Pencil } from './Pencil';
import { ErrorHandling, Settings } from './Settings';

import './Grid.css'


function handleCellUpdates(
  puzzle: Puzzle,
  currentCell: number,
  settings: Settings,
  setErrorCells: Dispatch<SetStateAction<Set<number>>>,
  setIsCompleted: Dispatch<SetStateAction<boolean>>,
) {
  const isComplete = puzzle.is_complete();
  setIsCompleted(isComplete);
  if (isComplete) return;

  puzzle.update_guesses(settings.autoPencil);
  let badCells = puzzle.verify(settings.errorHandling === ErrorHandling.ShowFalse || settings.errorHandling === ErrorHandling.BlockFalse);
  if (!badCells.length) {
    setErrorCells(new Set());
    return;
  }
  if (settings.errorHandling === ErrorHandling.ShowFalse || settings.errorHandling === ErrorHandling.ShowInconsistent) {
    // we could also do refs[badCell].current?.setCustomValidity("Duplicate") but that
    // doesn't work for readonly cells so we have to do a custom style instead
    setErrorCells(new Set(badCells));
  } else if (settings.errorHandling === ErrorHandling.BlockFalse || settings.errorHandling === ErrorHandling.BlockInconsistent) {
    puzzle.set_value(currentCell, 0);
  }
}

// TODO: use svg for lines overlay

export default function Grid(props: {puzzle: Puzzle, settings: Settings}) {
  const [highlightNumber, setHighlightNumber] = useState(0);
  const [midPencil, setMidPencil] = useState(new Array(props.puzzle.values.length));
  const [guessMode, setGuessMode] = useState(false);
  const [isCompleted, setIsCompleted] = useState(false);
  const [errorCells, setErrorCells] = useState(new Set<number>());
  const rerender = useReducer(x => x + 1, 0)[1];
  const refs: RefObject<HTMLInputElement>[] = [];

  const rows = [];
  const fixedValues = props.puzzle.fixed_values;
  const guesses = props.puzzle.guesses;
  const values = props.puzzle.values;
  for (let row = 0; row < props.puzzle.size; row++) {
    const col = [];
    for (let cell = props.puzzle.size * row; cell < props.puzzle.size * (row + 1); cell++) {
      const color = '';
      const fixed = fixedValues[cell];
      const value = values[cell];

      refs.push(createRef<HTMLInputElement>());
       
      const errorStyle = (errorCells.has(cell) ? 'error' : '');
      col.push(<td key={cell} className={`${errorStyle}`} style={{backgroundColor: color || ''}}>
        <Pencil pencil={value ? '' : guesses[cell]} midPencil={midPencil[cell]} highlightNumber={highlightNumber} />
        <input 
          ref={refs.slice(-1)[0]}
          className={`num-box ${value === highlightNumber ? 'highlight' : ''}`}
          inputMode="numeric"
          pattern="[1-9.,]"
          maxLength={1}
          readOnly={fixed}
          onKeyDown={(evt) => {
            const pencilNum = '!@#$%^&*('.indexOf(evt.key) + 1;
            if (pencilNum !== 0 && !fixed) {
              if (value) return;
              props.puzzle.set_guess(cell, pencilNum);
              if (props.settings.highlight) setHighlightNumber(pencilNum);
              rerender();
            } else if (/[1-9]/.test(evt.key) && !fixed) {
              if (guessMode) {
                props.puzzle.set_guess(cell, parseInt(evt.key));
                if (props.settings.highlight) setHighlightNumber(parseInt(evt.key));
                rerender();
                return;
              }
              props.puzzle.set_value(cell, parseInt(evt.key));
              handleCellUpdates(props.puzzle, cell, props.settings, setErrorCells, setIsCompleted);
              if (props.settings.highlight) setHighlightNumber(parseInt(evt.key));
              rerender();
            } else if (evt.key === '-' && highlightNumber && !fixed) {
              // NOTE: this doesn't work on Mobile Chrome
              if (guessMode) {
                props.puzzle.set_guess(cell, highlightNumber);
                rerender();
                return;
              }
              if (values[cell] === highlightNumber) {
                props.puzzle.set_value(cell, 0);
              } else {
                props.puzzle.set_value(cell, highlightNumber);
              }
              handleCellUpdates(props.puzzle, cell, props.settings, setErrorCells, setIsCompleted);
              rerender();
            } else if (evt.key === '_' && !fixed) {
              // NOTE: this also doesn't work on Mobile Chrome
              props.puzzle.set_guess(cell, highlightNumber);
              rerender();
            } else if (evt.key === '0') {
              setHighlightNumber(0);
            } else if (!evt.shiftKey && evt.key === 'Backspace' && !fixed) {
              if (guessMode) {
                props.puzzle.erase_guess(cell);
                rerender();
                return;
              }
              props.puzzle.set_value(cell, 0);
              handleCellUpdates(props.puzzle, cell, props.settings, setErrorCells, setIsCompleted);
              rerender();
            } else if (evt.shiftKey && evt.key === 'Backspace' && !fixed) {
              props.puzzle.erase_guess(cell);
              rerender();
            } else if (evt.key === 'ArrowDown' && cell < 72) {
              refs[cell + 9].current?.focus();
            } else if (evt.key === 'ArrowDown') {
              refs[cell - 72].current?.focus();
            } else if (evt.key === 'ArrowUp' && cell > 8) {
              refs[cell - 9].current?.focus();
            } else if (evt.key === 'ArrowUp') {
              refs[cell + 72].current?.focus();
            } else if (evt.key === 'ArrowRight' && cell % 9 < 8) {
              refs[cell + 1].current?.focus();
            } else if (evt.key === 'ArrowRight') {
              refs[cell - 8].current?.focus();
            } else if (evt.key === 'ArrowLeft' && cell % 9 > 0) {
              refs[cell - 1].current?.focus();
            } else if (evt.key === 'ArrowLeft') {
              refs[cell + 8].current?.focus();
            }
          }}
          onChange={(evt) => {
            // Chrome mobile has a bug where these keys register identically in onKeyDown
            // so we have to check here: https://issues.chromium.org/issues/41368867
            if ((evt.target.value === ')' || evt.target.value === ',') && !fixed) {
              const newMidPencil = [...midPencil];
              newMidPencil[cell] = !newMidPencil[cell]
              setMidPencil(newMidPencil);
            } else if (evt.target.value === '.') {
              setGuessMode(!guessMode);
            }
            evt.preventDefault();
            evt.stopPropagation();
          }}
          value={value ? `${value}` : ''}
        />
      </td>);
    }
    rows.push(<tr key={row}>{col}</tr>);
  }

  const numberSelector: ReactNode[] = [];
  for (let n = 1; n <= props.puzzle.size; n++) {
    numberSelector.push(<td
      key={n}
      onClick={() => setHighlightNumber(highlightNumber == n ? 0 : n)}
      className={highlightNumber == n ? 'highlight' : ''}
    >
      {n}
    </td>);
  }

  let completeBanner = (<td colSpan={props.puzzle.size}> {'\uD83C\uDF89'} Congratulations!</td>);

  return (
    <div className="grid-table">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height={30*9+8}
        width={30*9+8}
        className="overlay"
        style={{}}
      />
      <table className={`${guessMode ? 'guess-mode' : ''}`}>
        <tbody>
          {rows}
        </tbody>
        <tfoot>
          <tr>
            {isCompleted ? completeBanner : numberSelector}
          </tr>
        </tfoot>
      </table>
    </div>
  )
}
