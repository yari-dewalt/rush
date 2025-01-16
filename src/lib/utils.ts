export function convertBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return '0 B';
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
  const k = 1000;
  
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  const value = bytes / Math.pow(k, i);
  
  return `${value.toFixed(decimals)} ${units[i]}`;
}
