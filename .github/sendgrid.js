#! /usr/bin/env node

const fs = require('fs');

const sgMail = require('@sendgrid/mail');
sgMail.setApiKey(process.env.SENDGRID_API_KEY);

const subjectBuffer = fs.readFileSync('subject.txt');
const bodyRawBuffer = fs.readFileSync('body.md');
const bodyHtmlBuffer = fs.readFileSync('body.html');

const now = new Date(Date.now());
const msg = {
  to: 'hockeybuggy@gmail.com',
  from: 'hockeybuggy+recurring@gmail.com',
  subject: subjectBuffer.toString(),
  text: bodyRawBuffer.toString(),
  html: bodyHtmlBuffer.toString(),
};

sgMail
  .send(msg)
  .then(() => console.log('Mail sent successfully'))
  .catch(error => console.error(error.toString()));
