import { type ClassValue, clsx } from 'clsx';

export function cn(...inputs: ClassValue[]) {
  return clsx(inputs);
}

export function formatPrice(price: number): string {
  return `${(price / 1_000_000_000).toFixed(2)} SUI`;
}

export function formatDate(date: string): string {
  return new Date(date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export function shortenAddress(address: string, chars = 4): string {
  if (address.length <= chars * 2 + 2) return address;
  return `${address.slice(0, chars + 2)}...${address.slice(-chars)}`;
}

export function extractVariablesFromText(text: string): string[] {
  const regex = /\{\{(\w+)\}\}/g;
  const matches = text.matchAll(regex);
  const variables = new Set<string>();
  for (const match of matches) {
    variables.add(match[1]);
  }
  return Array.from(variables);
}
