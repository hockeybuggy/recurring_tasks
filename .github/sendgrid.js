#! /usr/bin/env node

const fs = require('fs');

const sgMail = require('@sendgrid/mail');
sgMail.setApiKey(process.env.SENDGRID_API_KEY);

const programOutput = fs.readFileSync('output.txt');

const msg = {
  to: 'hockeybuggy@gmail.com',
  from: 'hockeybuggy+recurring@gmail.com',
  subject: 'Recurring tasks for YYYY-MM-DD',
  text: programOutput.toString(),
  html: `<p>${programOutput.toString()}</p>`,
};

sgMail
  .send(msg)
  .then(() => console.log('Mail sent successfully'))
  .catch(error => console.error(error.toString()));
