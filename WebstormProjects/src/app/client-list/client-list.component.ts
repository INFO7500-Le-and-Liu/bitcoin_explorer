// src/app/client-list/client-list.component.ts
import { Component, OnInit } from '@angular/core';
import { ClientService } from '../client.service';

interface Client {
  id: number;
  name: string;
  height: number;
}

@Component({
  selector: 'app-client-list',
  templateUrl: './client-list.component.html',
  styleUrls: ['./client-list.component.css']
})
export class ClientListComponent implements OnInit {
  clients: Client[] = [];

  constructor(private clientService: ClientService) {}

  ngOnInit(): void {
    this.clientService.getClients().subscribe(clients => {
      this.clients = clients;
    });
  }
}
