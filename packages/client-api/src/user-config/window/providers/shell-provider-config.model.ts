import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const ShellProviderConfigSchema = z.object({
  type: z.literal(ProviderType.UTIL),
});

export type ShellProviderConfig = z.infer<typeof ShellProviderConfigSchema>;
