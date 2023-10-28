import { CpuProviderOptions, CpuProviderOptionsSchema } from '~/user-config';
import { memoize } from '~/utils';

const DEFAULT = CpuProviderOptionsSchema.parse({});

export const createCpuProvider = memoize(
  (options: CpuProviderOptions = DEFAULT) => {
    return {
      usage: 0,
      temp: 0,
      frequency: 0,
    };
  },
);
