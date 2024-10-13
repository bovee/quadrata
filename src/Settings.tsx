import {
  useMantineColorScheme,
  Checkbox,
  MantineColorScheme,
  NativeSelect,
} from '@mantine/core';
import { Dispatch, SetStateAction } from 'react';


export enum ErrorHandling {
  Allow = 'allow',
  BlockFalse = 'block_false',
  BlockInconsistent = 'block_inconsistent',
  ShowFalse = 'show_false',
  ShowInconsistent = 'show_inconsistent',
}

// TODO: remove this once https://github.com/rustwasm/wasm-bindgen/issues/3057 is fixed
export enum AutoPencil {
  Never = 'never',
  OnlyRemove = 'onlyremove',
  Always = 'always',
  Snyder = 'synder',
}

export interface Settings {
  errorHandling: ErrorHandling;
  autoPencil: AutoPencil;
  highlight: boolean;
}

export const DEFAULT_SETTINGS: Settings = {
  errorHandling: ErrorHandling.BlockInconsistent,
  autoPencil: AutoPencil.Never,
  highlight: false,
};

export function SettingsPane(props: {settings: Settings, setSettings: Dispatch<SetStateAction<Settings>>}) {
  const { colorScheme, setColorScheme } = useMantineColorScheme();
  const { settings, setSettings } = props;

  return (<div>
    <NativeSelect
      label="Color Scheme"
      value={colorScheme}
      onChange={evt =>
        setColorScheme(evt.currentTarget.value as MantineColorScheme)
      }
      data={[
        { label: 'Light', value: 'light' },
        { label: 'Dark', value: 'dark' },
        { label: 'Auto', value: 'auto' },
      ]}
    />
    <NativeSelect
      label="Error Handling"
      value={settings.errorHandling}
      onChange={evt => {
        settings.errorHandling = evt.currentTarget.value as ErrorHandling;
        setSettings({...settings});
      }}
      data={[
        { label: 'Allow', value: ErrorHandling.Allow },
        { label: 'Block False', value: ErrorHandling.BlockFalse },
        { label: 'Block Inconsistent', value: ErrorHandling.BlockInconsistent },
        { label: 'Show False', value: ErrorHandling.ShowFalse },
        { label: 'Show Inconsistent', value: ErrorHandling.ShowInconsistent }
      ]}
    />
    <NativeSelect
      label="Automatically Pencil"
      value={settings.autoPencil}
      onChange={evt => {
        settings.autoPencil = evt.currentTarget.value as AutoPencil;
        setSettings({...settings});
      }}
      data={[
        { label: 'Never', value: AutoPencil.Never },
        { label: 'Allow', value: AutoPencil.Always },
        { label: 'Only Remove', value: AutoPencil.OnlyRemove },
        { label: 'Snyder', value: AutoPencil.Snyder },
      ]}
    />
    <br />
    <Checkbox
      label="Highlight active number"
      checked={settings.highlight}
      onChange={evt => {
        settings.highlight = evt.currentTarget.checked;
        setSettings({...settings});
      }}
    />
  </div>);
}
