export const useNumberFilter = (val) => {
  if(val >= 1000) val = (val/1000).toFixed(0) + 'K'
  if(val >= 1000000) val = (val/1000000).toFixed(0) + 'M'
  return $$(val)
}
