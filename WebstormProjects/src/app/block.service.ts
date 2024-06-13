//----block.service.ts
//----block.service.ts
import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Record } from './record.model';
import { catchError, map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class BlockService {
  private apiUrl = 'http://localhost:8080/blocks';

  constructor(private http: HttpClient) { }

  // getHeights(): Observable<number[]> {
  // getHeights(): Observable<number[]> {
  //   return this.http.get<any[]>(this.apiUrl).pipe(
  //     map(data => data.map(item => item.height))
  //     map(data => data.map(item => item.height))
  //   );
  // }
  getHeights(): Observable<Record[]> {
    return this.http.get<Record[]>(this.apiUrl);
  }
}
