function TypedArrayToHeap(arr) {
  const { name, BYTES_PER_ELEMENT } = arr.constructor;
  const prefix = name.charAt(0).replace(/I|B/, '');
  const heap = Module[`HEAP${prefix}${BYTES_PER_ELEMENT << 3}`];

  if (!heap) {
    throw new Error(`Unknow typed array ${heap}`);
  }

  const ptr = Module._malloc(arr.length * BYTES_PER_ELEMENT);
  heap.set(arr, ptr / BYTES_PER_ELEMENT);

  return ptr;
}

function runWasm(func, onload) {
  var script = document.createElement('script');
  script.onload = onload;
  script.src = func + '.js';
  document.body.appendChild(script);
}

function multiply(func, array, arr1, arr2, arr3) {
  const size = array.length;
  const time = [0, 0];

  time[0] = performance.now();

  Module.ccall(
    func,
    null,
    ['number', 'number', 'number', 'number'],
    [arr1, arr2, arr3, size]
  );

  time[1] = performance.now();

  const startPtr = arr1 / Int32Array.BYTES_PER_ELEMENT;
  const diff = time[1] - time[0];
  console.log(`${func}: ${diff} with ${size} length`);
}
