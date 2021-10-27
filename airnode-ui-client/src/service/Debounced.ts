export const Debounced: any = {
    queue: {},
    saveInterval: 500, // 10 seconds
  
    start: (id: any, callback: any, ms: any) => {
      if (typeof callback !== 'function') {
        throw new Error('Debounced.start: invalid arguments, at least callback expected');
      }
      const msValue = ms || 500;
      const timeoutId: any = Debounced.queue[id];
      clearTimeout(timeoutId);
      Debounced.queue[id] = setTimeout(() => {
        callback();
        delete Debounced.queue[id];
      }, msValue);
    },
  
    stop: (id: any) => {
      const timeoutId = Debounced.queue[id];
      if (timeoutId) {
        clearTimeout(timeoutId);
        delete Debounced.queue[id];
      }
    }
  };
  
  export default Debounced;
  