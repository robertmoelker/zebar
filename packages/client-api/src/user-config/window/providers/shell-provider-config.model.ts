import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const ShellProviderConfigSchema = z.object({
  type: z.literal(ProviderType.SHELL),

  /**
   * How often this component refreshes in milliseconds.
   */
  refresh_interval: z.coerce.number().default(60 * 60 * 1000),
});

export type ShellProviderConfig = z.infer<
  typeof ShellProviderConfigSchema
>;
