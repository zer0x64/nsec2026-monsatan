# monsatan-invoices

## Description

Simple IDOR vulnerability with sequential invoice numbers. Participants need to brute-force the invoice number to find the proof that Monsatan is a customer of Verdachem Industries. Meant to be easy, as it's the first stage of the track.

## Theme

In this track, the participants are tasked with finding proof that Monsatan is a customer of Verdachem Industries. This can be done by leaking a pesticide order invoice from Verdachem Industries' B2B website.

## Solutions

### Flag 1

The flag can be found in the address field of the Monsatan's invoice:
http://verdachem.ctf/invoice/10394

The participants can find this route by submitting an order. This redirects them to the invoice page, and sending a second one they can easily find that the IDs are sequential. They can then brute-force the invoice number and look for either monsatan or flag to find the right one.
