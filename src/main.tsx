import { ActionIcon, Drawer, MantineProvider, ThemeIcon } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { notifications, Notifications } from '@mantine/notifications';
import { useState, StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { useLocalStorage } from "@uidotdev/usehooks";

import { init, from_81, Puzzle } from '../quadrata/pkg/quadrata';
import Grid from './Grid.tsx'
import { AutoPencil, DEFAULT_SETTINGS, SettingsPane } from './Settings';
import './index.css'

import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';

init();

// TODO: store the current puzzle and state in local storage
const DATA = '016400000200009000400000062070230100100000003003087040960000005000800007000006820';
let setNewPuzzle: (data: Puzzle) => void;

function App() {
  const [opened, { open, close }] = useDisclosure(false);
  const [settings, setSettings] = useLocalStorage('settings', DEFAULT_SETTINGS);
  const [puzzle, setPuzzle] = useState(formatPuzzle(from_81(DATA)));

  function formatPuzzle(puzzle: Puzzle): Puzzle {
    elapsed = 0;
    currentStart = new Date().getTime();
    if (settings.autoPencil === AutoPencil.OnlyRemove) {
      puzzle.update_guesses(AutoPencil.Always);
    } else {
      puzzle.update_guesses(settings.autoPencil);
    }
    return puzzle;
  }
  setNewPuzzle = (data) => {
    setPuzzle(formatPuzzle(data));
  };

  return (
    <>
      <div className="controls" style={{ textAlign: 'right'}}>
        <ActionIcon onClick={open} aria-label="Settings" variant="subtle" size="xl">
          ⚙
        </ActionIcon>
      </div>
      <Drawer position="right" opened={opened} onClose={close} title="Settings">
        <SettingsPane settings={settings} setSettings={setSettings} />
      </Drawer>
      <Grid puzzle={puzzle} settings={settings} />
    </>
  );
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <MantineProvider>
      <App />
      <Notifications />
    </MantineProvider>
  </StrictMode>,
)

document.addEventListener('paste', (event) => {
  event.preventDefault();
  let text = (event.clipboardData || (window as any).clipboardData).getData("text");
  try {
    let puzzle = from_81(text.trim());
    setNewPuzzle(puzzle);
  } catch (error) {
    notifications.show({
      title: 'Error importing puzzle',
      color: 'red',
      icon: <ThemeIcon variant="white" color="red">❗</ThemeIcon>,
      message: (error as Error).message,
    });
    return;
  }
  notifications.show({
    title: 'Puzzle imported',
    message: 'Puzzle imported',
  });
});

let elapsed = 0;
let currentStart = new Date().getTime();

document.addEventListener('visibilitychange', () => {
  if (document.hidden) {
    // pause timer
    elapsed += new Date().getTime() - currentStart;
  } else {
    // resume timer
    currentStart = new Date().getTime();
  }
}, false);
