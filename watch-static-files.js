const browerSync = require('browser-sync').create();

browerSync.init({
  server: {
    baseDir: 'html'
  },
  files: ['html/**/*,*'],
  notify: false,
  open: false,
  port: 8080
});

browerSync.watch('html/**/*.*').on('change', browerSync.reload);
