import type { Owner } from 'solid-js';

import type { ShellProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface ShellVariables {
  response: string;
}

export async function createShellProvider(
  config: ShellProviderConfig,
  owner: Owner,
) {
  const shellVariables = await createProviderListener<
    ShellProviderConfig,
    ShellVariables
  >(config, owner);

  return {
    get response() {
      return shellVariables().response;
    },
    // get usage() {
    //   return memoryVariables().usage;
    // },
    // get freeMemory() {
    //   return memoryVariables().freeMemory;
    // },
    // get usedMemory() {
    //   return memoryVariables().usedMemory;
    // },
    // get totalMemory() {
    //   return memoryVariables().totalMemory;
    // },
    // get freeSwap() {
    //   return memoryVariables().freeSwap;
    // },
    // get usedSwap() {
    //   return memoryVariables().usedSwap;
    // },
    // get totalSwap() {
    //   return memoryVariables().totalSwap;
    // },
  };
}
