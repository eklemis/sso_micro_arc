import { PrismaClient } from '@prisma/client';
import { writable } from 'svelte/store';

export const prisma_client = writable(new PrismaClient());
