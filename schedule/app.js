import { CronJob } from 'cron';
import 'dotenv/config';
import express from 'express';

const app = express();

app.set('port', process.env.PORT || 5544);

const mainExpression = '*/14 * * * *';
const twoSecs = '*/10 * * * * *';

const job = new CronJob(mainExpression, async function () {
  const req = await fetch(process.env.JOLLOF_HEALTH_CHECK);
  const res = await req.json();

  console.log(res);
  console.log('Ya!');
});

job.start();

app.listen(app.get('port'), err => {
  if (err) console.log(`Server failure due to ${err.message}`);
  console.log(`Server running on port ${app.get('port')}`);
});
