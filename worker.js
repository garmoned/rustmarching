
self.onmessage = event => {
    let initialised = wasm_bindgen(...event.data).catch(err => {
      // Propagate to main `onerror`:
      setTimeout(() => {
        throw err;
      });
      // Rethrow to keep promise rejected and prevent execution of further commands:
      throw err;
    });
  
    self.onmessage = async event => {
      // This will queue further commands up until the module is fully initialised:
      await initialised;
      wasm_bindgen.child_entry_point(event.data);
    };
  };