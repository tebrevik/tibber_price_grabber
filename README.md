# tibber_price_grabber
## About

tibber_price_grabber is a small project that pulls prices from the Tibber API and:
* publish CloudEvents to Nats for use by other applications.
* display the prices to the terminal as a list
* display the prices to the terminal as a prioritizeed list of hours where the electricity is cheapest

## Usage

Find your API Access token at developer.tibber.com.


### To find home id's
```
$ TIBBER_TOKEN=$TOKEN ./tibber_price_grabber --mode ListHomes
```

### To print list of hourly prices
```
$ TIBBER_TOKEN=$TOKEN TIBBER_HOME_ID=$HOME_ID ./tibber_price_grabber --mode List 
hour: 2022-11-02T17:00:00+01:00, price: 1.7822
hour: 2022-11-02T18:00:00+01:00, price: 1.7428
hour: 2022-11-02T19:00:00+01:00, price: 1.3012
hour: 2022-11-02T20:00:00+01:00, price: 1.1818
hour: 2022-11-02T21:00:00+01:00, price: 1.043
hour: 2022-11-02T22:00:00+01:00, price: 1.104
hour: 2022-11-02T23:00:00+01:00, price: 0.9838
hour: 2022-11-03T00:00:00+01:00, price: 0.8657
hour: 2022-11-03T01:00:00+01:00, price: 0.6032
hour: 2022-11-03T02:00:00+01:00, price: 0.5086
hour: 2022-11-03T03:00:00+01:00, price: 0.5247
hour: 2022-11-03T04:00:00+01:00, price: 0.5536
hour: 2022-11-03T05:00:00+01:00, price: 0.6692
hour: 2022-11-03T06:00:00+01:00, price: 0.9828
hour: 2022-11-03T07:00:00+01:00, price: 1.1262
hour: 2022-11-03T08:00:00+01:00, price: 1.0856
hour: 2022-11-03T09:00:00+01:00, price: 1.1605
hour: 2022-11-03T10:00:00+01:00, price: 1.1778
hour: 2022-11-03T11:00:00+01:00, price: 1.3228
hour: 2022-11-03T12:00:00+01:00, price: 1.2981
hour: 2022-11-03T13:00:00+01:00, price: 1.3778
hour: 2022-11-03T14:00:00+01:00, price: 1.3791
hour: 2022-11-03T15:00:00+01:00, price: 1.3274
hour: 2022-11-03T16:00:00+01:00, price: 1.3915
hour: 2022-11-03T17:00:00+01:00, price: 1.493
hour: 2022-11-03T18:00:00+01:00, price: 1.3815
hour: 2022-11-03T19:00:00+01:00, price: 1.2373
hour: 2022-11-03T20:00:00+01:00, price: 1.2012
hour: 2022-11-03T21:00:00+01:00, price: 1.1914
hour: 2022-11-03T22:00:00+01:00, price: 1.2392
hour: 2022-11-03T23:00:00+01:00, price: 1.0851
2022-11-02 - avg: 0.900, max: 1.782, min: 0.210
2022-11-03 - avg: 1.091, max: 1.493, min: 0.509
```

### To print prioritized hours
```
$ TIBBER_TOKEN=$TOKEN TIBBER_HOME_ID=$HOME_ID ./tibber_price_grabber --mode Priority --periode-hours 9 --number-of-elements-prioritized 2
hour: 2022-11-02T03:00:00+01:00, price: 0.2099
hour: 2022-11-02T02:00:00+01:00, price: 0.2383
---
hour: 2022-11-02T11:00:00+01:00, price: 0.5695
hour: 2022-11-02T12:00:00+01:00, price: 0.586
---
hour: 2022-11-03T02:00:00+01:00, price: 0.5086
hour: 2022-11-03T01:00:00+01:00, price: 0.6032
---
hour: 2022-11-03T03:00:00+01:00, price: 0.5247
hour: 2022-11-03T04:00:00+01:00, price: 0.5536
---
hour: 2022-11-03T20:00:00+01:00, price: 1.2012
hour: 2022-11-03T19:00:00+01:00, price: 1.2373
---
hour: 2022-11-03T23:00:00+01:00, price: 1.0851
hour: 2022-11-03T21:00:00+01:00, price: 1.1914
---
```

### To publish prices to Nats using CloudEvents
```
$ TIBBER_TOKEN=$TOKEN TIBBER_HOME_ID=$HOME_ID ./tibber_price_grabber --mode CloudEvents --server-nats localhost:4222 --subject-nats foo
published
```

