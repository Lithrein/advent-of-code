#! /usr/bin/env racket
#lang racket

(define (load-map-from file-name)
  (list->vector (map
    (lambda (str) (list->vector (string->list str)))
      (file->lines file-name))))

(module+ test
  (require rackunit)
  (define raw-sample
#<<DATA
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
DATA
)
  (define sample (list->vector
                   (map list->vector
                     (map string->list
                       (string-split raw-sample "\n"))))))

(define (count-tree array dx dy)
  (define height (vector-length array))
  (define width (vector-length (vector-ref array 0)))
  (define (walk-cnt i j)
    (cond
      [ (< i height)
           (+ (walk-cnt (+ i dx) (modulo (+ j dy) width))
           (if (char=? (vector-ref (vector-ref array i) j) #\#) 1 0))]
      [else 0]))
  (walk-cnt 0 0))

(define (day03 array slopes)
  (define/match (aux slope acc)
    [(`(,dx ,dy) _) (* acc (count-tree array dx dy))])
  (foldl aux 1 slopes))

(define (part1 array)
  (day03 array '((1 3))))

(module+ test
  (check-equal? (part2 sample) 336))

(define (part2 array)
  (day03 array '((1 1) (1 3) (1 5) (1 7) (2 1))))

(module+ test
  (check-equal? (part1 sample) 7))

(module+ main
  (define input (load-map-from "../inputs/day03"))
  (printf "part1: ~a~npart2: ~a~n" (part1 input) (part2 input)))
